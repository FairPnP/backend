# ==============================================================================
# S3 Bucket for User Generated Content

# tfsec:ignore:aws-s3-block-public-acls
# tfsec:ignore:aws-s3-block-public-policy
# tfsec:ignore:aws-s3-ignore-public-acls
# tfsec:ignore:aws-s3-no-public-buckets
# tfsec:ignore:aws-s3-specify-public-access-block
# tfsec:ignore:aws-s3-enable-bucket-logging
# tfsec:ignore:aws-s3-enable-versioning
# tfsec:ignore:aws-s3-enable-bucket-encryption
# tfsec:ignore:aws-s3-encryption-customer-key
resource "aws_s3_bucket" "user_content" {
  bucket = "${local.namespace}-user-content"

  tags = local.tags
}

# ==============================================================================
# ACLs - Public Access

# tfsec:ignore:aws-s3-block-public-acls
# tfsec:ignore:aws-s3-ignore-public-acls
# tfsec:ignore:aws-s3-block-public-policy
# tfsec:ignore:aws-s3-no-public-buckets
resource "aws_s3_bucket_public_access_block" "user_content_public_access_block" {
  bucket = aws_s3_bucket.user_content.id

  block_public_acls       = false
  ignore_public_acls      = false
  block_public_policy     = false
  restrict_public_buckets = false
}

# ==============================================================================
# Bucket Policy - Public Read Access

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

# ==============================================================================
# IAM Policy for Upload

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
    ]
  })
}

# ==============================================================================
# Outputs

output "user_content_bucket_name" {
  value = aws_s3_bucket.user_content.bucket
}
