resource "cloudflare_zone_settings_override" "this" {
  zone_id = var.zone_id

  settings {
    ssl                      = var.ssl_mode
    always_use_https         = "on"
    security_level           = var.security_level
    automatic_https_rewrites = "on"
    min_tls_version          = var.min_tls_version
  }
}
