#tfsec:ignore:aws-s3-encryption-customer-key
#tfsec:ignore:aws-s3-enable-bucket-logging
resource "aws_s3_bucket" "tf-state" {
  bucket_prefix = "fairpnp-dev-terraform-state-"

  server_side_encryption_configuration {
    rule {
      apply_server_side_encryption_by_default {
        sse_algorithm = "AES256"
      }
    }
  }
}

resource "aws_s3_bucket_public_access_block" "tf-state" {
  bucket                  = aws_s3_bucket.tf-state.id
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_versioning" "tf-state" {
  bucket = aws_s3_bucket.tf-state.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_kms_key" "dynamo-tf-state-lock" {
  description         = "KMS key for tf DynamoDB table"
  enable_key_rotation = true
}

resource "aws_dynamodb_table" "dynamodb-tf-state-lock" {
  name           = "fairpnp-dev-tf-state-lock-dynamo"
  hash_key       = "LockID"
  read_capacity  = 4
  write_capacity = 4

  attribute {
    name = "LockID"
    type = "S"
  }

  server_side_encryption {
    enabled     = true
    kms_key_arn = aws_kms_key.dynamo-tf-state-lock.arn
  }

  point_in_time_recovery {
    enabled = true
  }
}
