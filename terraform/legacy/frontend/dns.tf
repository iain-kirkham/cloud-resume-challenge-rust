resource "aws_route53_record" "www" {
  name    = "iainkirkham.dev"
  type    = "A"
  zone_id = var.route53_zone_id
  alias {
    evaluate_target_health = false
    name                   = aws_cloudfront_distribution.cloud_resume_challenge_distribution.domain_name
    zone_id                = "Z2FDTNDATAQYW2"
  }
}
