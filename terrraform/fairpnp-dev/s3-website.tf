locals {
  websites = {
    "www.${local.cloudflare.domain}" = {
      bucket = "www.${local.cloudflare.domain}"
      cname  = "www.${local.cloudflare.domain}"
      redirect = {
        hostname = local.cloudflare.domain
        protocol = "https"
      },
      cloudflare_priority = 1
    },
    (local.cloudflare.domain) = {
      bucket              = local.cloudflare.domain
      cname               = local.cloudflare.domain
      redirect            = null
      cloudflare_priority = 2
    },
  }
}
// ============================================================================
// S3 Bucket

#tfsec:ignore:aws-s3-enable-bucket-logging
resource "aws_s3_bucket" "website" {
  for_each = local.websites

  bucket = each.value.bucket

  tags = local.tags
}

resource "aws_s3_bucket_website_configuration" "website" {
  for_each = local.websites

  bucket = aws_s3_bucket.website[each.key].id

  index_document {
    suffix = "index.html"
  }

  error_document {
    key = "error.html"
  }

  dynamic "routing_rule" {
    for_each = each.value.redirect != null ? [1] : []

    content {
      redirect {
        host_name = each.value.redirect.hostname
        protocol  = each.value.redirect.protocol
      }

      condition {
        http_error_code_returned_equals = 404
      }
    }
  }
}

#tfsec:ignore:aws-s3-encryption-customer-key
resource "aws_s3_bucket_server_side_encryption_configuration" "website" {
  for_each = local.websites

  bucket = aws_s3_bucket.website[each.key].id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_versioning" "website" {
  for_each = local.websites

  bucket = aws_s3_bucket.website[each.key].id

  versioning_configuration {
    status = "Enabled"
  }
}

// ============================================================================
// S3 Bucket Policy

#tfsec:ignore:aws-s3-block-public-acls
#tfsec:ignore:aws-s3-ignore-public-acls
#tfsec:ignore:aws-s3-block-public-policy
#tfsec:ignore:aws-s3-no-public-buckets
resource "aws_s3_bucket_public_access_block" "website_public_access_block" {
  for_each = local.websites

  bucket = aws_s3_bucket.website[each.key].id

  block_public_acls       = false
  ignore_public_acls      = false
  block_public_policy     = false
  restrict_public_buckets = false
}

resource "aws_s3_bucket_policy" "website_policy" {
  for_each = local.websites

  bucket = aws_s3_bucket.website[each.key].id

  # only allow access from cloudflare IPs
  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action    = ["s3:GetObject"],
        Effect    = "Allow",
        Resource  = "${aws_s3_bucket.website[each.key].arn}/*",
        Principal = "*",
        Condition = {
          IpAddress : {
            "aws:SourceIp" : data.cloudflare_ip_ranges.current.ipv4_cidr_blocks,
          }
        }
      },
    ],
  })
}
