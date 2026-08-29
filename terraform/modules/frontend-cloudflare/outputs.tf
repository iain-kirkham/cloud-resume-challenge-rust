output "zone_settings_id" {
  description = "ID of the zone settings override resource"
  value       = cloudflare_zone_settings_override.this.id
}
