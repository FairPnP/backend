locals {
  iam_users = {
  }

  user_roles = {
  }

  assumable_roles = {
    ec2_api_service = {
      policies = []
      service  = "ec2.amazonaws.com"
    }
  }

  secret_iam_users = [for k, v in local.iam_users : k if v.generate_secret]
}

# ==============================================================================
# IAM User config

#tfsec:ignore:aws-iam-no-user-attached-policies
resource "aws_iam_user" "users" {
  for_each = local.iam_users

  name = "${local.namespace}-${each.key}"

  tags = local.tags
}

resource "aws_iam_access_key" "github_user_key" {
  for_each = { for k in local.secret_iam_users : k => aws_iam_user.users[k] }

  user = each.value.name
}

# ==============================================================================
# IAM Role config

resource "aws_iam_role" "user_roles" {
  for_each = local.user_roles

  name = "${local.namespace}-${each.key}"
  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action = "sts:AssumeRole",
        Effect = "Allow",
        Principal = {
          AWS = "arn:aws:iam::${data.aws_caller_identity.current.account_id}:root"
        }
      }
    ]
  })

  tags = local.tags
}

resource "aws_iam_role" "assumable_roles" {
  for_each = local.assumable_roles

  name = "${local.namespace}-${each.key}"
  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action = "sts:AssumeRole",
        Effect = "Allow",
        Principal = {
          Service = each.value.service
        }
      }
    ]
  })
}

resource "aws_iam_user_policy" "user_assume_role_policy" {
  for_each = local.iam_users

  name = "${local.namespace}-${each.key}-assume-role-policy"
  user = aws_iam_user.users[each.key].name

  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      for role in each.value.roles : {
        Effect   = "Allow",
        Action   = "sts:AssumeRole",
        Resource = aws_iam_role.user_roles[role].arn
      }
    ]
  })
}


# ==============================================================================
# IAM Policy config

resource "aws_iam_role_policy_attachment" "role_policies" {
  for_each = { for _, value in flatten([
    for role, details in local.user_roles : [
      for idx, arn in details.policies : {
        idx        = idx
        role       = role
        policy_arn = arn
      }
    ]
  ]) : "${value.role}-${value.idx}" => value }

  role       = aws_iam_role.user_roles[each.value.role].name
  policy_arn = each.value.policy_arn
}

resource "aws_iam_role_policy_attachment" "service_role_policies" {
  for_each = { for _, value in flatten([
    for role, details in local.assumable_roles : [
      for idx, arn in details.policies : {
        idx        = idx
        role       = role
        policy_arn = arn
      }
    ]
  ]) : "${value.role}-${value.idx}" => value }

  role       = aws_iam_role.assumable_roles[each.value.role].name
  policy_arn = each.value.policy_arn
}

# ==============================================================================
# Output

output "iam_user_secrets" {
  value = {
    for k in local.secret_iam_users : k => {
      access_key = aws_iam_access_key.github_user_key[k].id
      secret     = aws_iam_access_key.github_user_key[k].secret
    }
  }
  sensitive = true
}
