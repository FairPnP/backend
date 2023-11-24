locals {
  user_pool = {
    name = local.namespace,
    callback_urls = [
      "http://localhost:3000/oauth2/callback",
      "https://www.getpostman.com/oauth2/callback",
      "https://oauth.pstmn.io/v1/callback",
    ],
    logout_urls = [
      "http://localhost:3000/oauth2/logout",
    ],
  }
}

# ==============================================================================
# User Pool config

resource "aws_cognito_user_pool" "customer_user_pool" {
  name = local.user_pool.name
  tags = local.tags

  password_policy {
    temporary_password_validity_days = 7
    minimum_length                   = 8
    require_lowercase                = true
    require_numbers                  = true
    require_symbols                  = true
    require_uppercase                = true
  }

  schema {
    attribute_data_type = "String"
    name                = "email"
    required            = true
    mutable             = true

    string_attribute_constraints {
      min_length = 0
      max_length = 2048
    }
  }

  auto_verified_attributes = ["email"]
  username_attributes      = ["email"]

  username_configuration {
    case_sensitive = false
  }
}

resource "aws_cognito_user_pool_domain" "customer_user_pool_domain" {
  domain       = local.user_pool.name
  user_pool_id = aws_cognito_user_pool.customer_user_pool.id
}

# ==============================================================================
# User Pool Client config

resource "aws_cognito_user_pool_client" "customer_user_pool_client" {
  name         = "${local.user_pool.name}-client"
  user_pool_id = aws_cognito_user_pool.customer_user_pool.id

  supported_identity_providers = ["COGNITO"]

  # OAuth settings
  allowed_oauth_flows                  = ["code", "implicit"]
  allowed_oauth_scopes                 = ["email", "openid", "profile"]
  allowed_oauth_flows_user_pool_client = true
  callback_urls                        = local.user_pool.callback_urls
  logout_urls                          = local.user_pool.logout_urls

  # Token validity settings 
  access_token_validity  = 1  # in hours
  refresh_token_validity = 30 # in days

  # To prevent client secret creation
  generate_secret = false
}

# ==============================================================================
# Outputs

output "cognito" {
  value = {
    user_pool_id       = aws_cognito_user_pool.customer_user_pool.id
    client_id          = aws_cognito_user_pool_client.customer_user_pool_client.id
    implicit_login_url = "https://${aws_cognito_user_pool_domain.customer_user_pool_domain.domain}.auth.${local.region}.amazoncognito.com/login?response_type=token&client_id=${aws_cognito_user_pool_client.customer_user_pool_client.id}&redirect_uri=${local.user_pool.callback_urls[0]}"
  }
}
