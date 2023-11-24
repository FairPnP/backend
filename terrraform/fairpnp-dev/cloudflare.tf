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
# DNS record

resource "cloudflare_record" "api" {
  zone_id = local.cloudflare.zone_id
  name    = local.api_subdomain
  value   = aws_lb.api_nlb.dns_name
  type    = "CNAME"
  ttl     = 1
  proxied = true
}
