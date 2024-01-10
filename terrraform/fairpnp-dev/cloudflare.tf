locals {
  cloudflare = {
    zone_id = "820cfc30193f53a04775ebc97d9236d2"
    domain  = "fairpnp.com"
  }

  api_subdomain   = "api-dev"
  api_full_domain = "${local.api_subdomain}.${local.cloudflare.domain}"
}

data "cloudflare_ip_ranges" "current" {}

# ==============================================================================
# DNS api server

resource "cloudflare_record" "api" {
  zone_id = local.cloudflare.zone_id
  name    = local.api_subdomain
  value   = aws_lb.api_nlb.dns_name
  type    = "CNAME"
  ttl     = 1
  proxied = true
}

# ==============================================================================
# DNS website

resource "cloudflare_record" "website" {
  for_each = local.websites

  zone_id = local.cloudflare.zone_id
  name    = each.value.cname
  value   = aws_s3_bucket_website_configuration.website[each.key].website_endpoint
  type    = "CNAME"
  proxied = true
}

# ==============================================================================
# Page Rules

resource "cloudflare_page_rule" "website" {
  for_each = local.websites

  zone_id  = local.cloudflare.zone_id
  target   = "${each.value.cname}/*"
  priority = each.value.cloudflare_priority

  actions {
    ssl = "flexible"
  }
}

// ==============================================================================
// Outputs

output "api_url" {
  value = "https://${local.api_full_domain}"
}

output "website_urls" {
  value = [
    for website in local.websites : "https://${website.cname}"
  ]
}
