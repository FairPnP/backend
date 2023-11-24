
terraform {

  required_version = "= 1.5.2"
  backend "s3" {
    bucket         = "fairpnp-dev-terraform-state-20231124033757230800000001"
    dynamodb_table = "fairpnp-dev-tf-state-lock-dynamo"
    key            = "fairpnp-dev-terraform"
    region         = "us-east-2"
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.5.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
    tls = {
      source  = "hashicorp/tls"
      version = "4.0.4"
    }
  }
}

# Configure the AWS Provider
provider "aws" {
  region  = local.region
  profile = local.profile
}

# Configure the ACM cert for Cloud Front
# ACM must be us-east-1: https://github.com/cloudposse/terraform-aws-cloudfront-s3-cdn/issues/55
provider "aws" {
  alias   = "cf-acm"
  region  = "us-east-1"
  profile = local.profile
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}
