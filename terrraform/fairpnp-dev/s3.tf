# ==============================================================================
# S3 Bucket for User Generated Content

# tfsec:ignore:aws-s3-block-public-acls
# tfsec:ignore:aws-s3-block-public-policy
# tfsec:ignore:aws-s3-ignore-public-acls
# tfsec:ignore:aws-s3-no-public-buckets
# tfsec:ignore:aws-s3-specify-public-access-block
# tfsec:ignore:aws-s3-enable-bucket-logging
# tfsec:ignore:aws-s3-enable-versioning
resource "aws_s3_bucket" "user_content" {
  bucket = "${local.namespace}-user-content"

  tags = local.tags
}

resource "aws_kms_key" "user_content_kms_key" {
  enable_key_rotation = true
}

resource "aws_s3_bucket_server_side_encryption_configuration" "user_content_encryption" {
  bucket = aws_s3_bucket.user_content.id

  rule {
    apply_server_side_encryption_by_default {
      kms_master_key_id = aws_kms_key.user_content_kms_key.arn
      sse_algorithm     = "aws:kms"
    }
  }
}
# ==============================================================================
# ACLs

# tfsec:ignore:aws-s3-block-public-policy
resource "aws_s3_bucket_public_access_block" "user_content_public_access_block" {
  bucket = aws_s3_bucket.user_content.id

  # Set to false to allow public read access via bucket policy
  block_public_policy = false

  block_public_acls       = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

# ==============================================================================
# Bucket Policy

resource "aws_s3_bucket_policy" "user_content_policy" {
  bucket = aws_s3_bucket.user_content.id

  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action    = ["s3:GetObject"],
        Effect    = "Allow",
        Resource  = "${aws_s3_bucket.user_content.arn}/*",
        Principal = "*",
      },
    ],
  })
}

# tfsec:ignore:aws-iam-no-policy-wildcards
resource "aws_iam_policy" "s3_user_content_upload_policy" {
  name        = "${local.namespace}-s3-upload-policy"
  description = "Policy for uploading to S3 bucket"

  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action   = ["s3:PutObject"],
        Effect   = "Allow",
        Resource = "${aws_s3_bucket.user_content.arn}/*",
      },
      {
        Action   = ["s3:ListBucket"],
        Effect   = "Allow",
        Resource = aws_s3_bucket.user_content.arn,
      },
      {
        Action = [
          "kms:GenerateDataKey",
          "kms:Decrypt"
        ],
        Effect   = "Allow",
        Resource = aws_kms_key.user_content_kms_key.arn,
      },
    ]
  })
}

# ==============================================================================
# Outputs

output "user_content_bucket_name" {
  value = aws_s3_bucket.user_content.bucket
}
