resource "cloudflare_ruleset" "rate_limiting" {
  count = length(var.rate_limit_rules) > 0 ? 1 : 0

  zone_id = var.zone_id
  name    = "default"
  kind    = "zone"
  phase   = "http_ratelimit"

  dynamic "rules" {
    for_each = var.rate_limit_rules
    content {
      ref         = rules.value.ref
      description = rules.value.description
      expression  = rules.value.expression
      action      = rules.value.action

      ratelimit {
        characteristics     = rules.value.ratelimit.characteristics
        period              = rules.value.ratelimit.period
        requests_per_period = rules.value.ratelimit.requests_per_period
        mitigation_timeout  = rules.value.ratelimit.mitigation_timeout
      }
    }
  }
}

resource "cloudflare_ruleset" "firewall_custom" {
  count = length(var.firewall_rules) > 0 ? 1 : 0

  zone_id = var.zone_id
  name    = "default"
  kind    = "zone"
  phase   = "http_request_firewall_custom"

  dynamic "rules" {
    for_each = var.firewall_rules
    content {
      ref         = rules.value.ref
      description = rules.value.description
      expression  = rules.value.expression
      action      = rules.value.action
    }
  }
}
