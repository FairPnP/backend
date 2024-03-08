
terraform {
  # We DO NOT have a backend for the bucket state, this is as designed.
  # Once the buckets/dynamo are created we NEVER want to destroy them. As such by
  # not even having a state file, we cannot accidentally delete them.

  required_version = "= 1.5.2"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.50.0"
    }
    null = {
      source  = "hashicorp/null"
      version = "~> 3.0"
    }
  }
}

# Configure the AWS Provider
provider "aws" {
  region  = local.region
  profile = local.profile
}
