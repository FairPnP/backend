locals {
  iam_users = {
    api_service = {
      generate_secret = true
      groups          = ["s3_user_content_upload"]
    },
    github_website_ci = {
      generate_secret = true
      groups          = ["website_write"]
    }
  }

  user_groups = {
    s3_user_content_upload = {
      policies = [
        aws_iam_policy.s3_user_content_upload_policy.arn
      ]
    },
    website_write = {
      policies = [
        aws_iam_policy.website_rw_policy[local.cloudflare.domain].arn
      ]
    }
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

resource "aws_iam_access_key" "secret_user_key" {
  for_each = { for k in local.secret_iam_users : k => aws_iam_user.users[k] }

  user = each.value.name
}

# ==============================================================================
# IAM Group config

# tfsec:ignore:aws-iam-enforce-group-mfa
resource "aws_iam_group" "user_groups" {
  for_each = local.user_groups

  name = "${local.namespace}-${each.key}"
}

resource "aws_iam_group_policy_attachment" "user_group_attach" {
  for_each = { for _, value in flatten([
    for group, details in local.user_groups : [
      for idx, arn in details.policies : {
        idx        = idx
        group      = group
        policy_arn = arn
      }
    ]
  ]) : "${value.group}-${value.idx}" => value }

  group      = aws_iam_group.user_groups[each.value.group].name
  policy_arn = each.value.policy_arn
}

resource "aws_iam_user_group_membership" "user_group_membership" {
  for_each = { for k, v in local.iam_users : k => v if length(v.groups) > 0 }

  user = aws_iam_user.users[each.key].name
  groups = [
    for g in each.value.groups : aws_iam_group.user_groups[g].name
  ]
}

# ==============================================================================
# IAM Assumable Role config

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

resource "aws_iam_role_policy_attachment" "assumable_role_policies" {
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
      access_key = aws_iam_access_key.secret_user_key[k].id
      secret     = aws_iam_access_key.secret_user_key[k].secret
    }
  }
  sensitive = true
}
