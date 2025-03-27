resource "aws_s3_bucket" "cloud_resume_bucket" {
  bucket = var.bucket_name
}

resource "aws_s3_bucket_acl" "main" {
  bucket = aws_s3_bucket.cloud_resume_bucket.id
  acl    = "private"
}


resource "aws_s3_bucket_policy" "cloudfront_oac_access" {
  bucket = aws_s3_bucket.cloud_resume_bucket.id
  policy = data.aws_iam_policy_document.cloudfront_oac_access.json
}

data "aws_iam_policy_document" "cloudfront_oac_access" {
  statement {
    principals {
      type        = "Service"
      identifiers = ["cloudfront.amazonaws.com"]
    }

    actions = [
      "s3:GetObject"
    ]

    resources = [
      aws_s3_bucket.cloud_resume_bucket.arn,
      "${aws_s3_bucket.cloud_resume_bucket.arn}/*"
    ]

    condition {
      test     = "StringEquals"
      variable = "AWS:SourceArn"
      values   = [aws_cloudfront_distribution.cloud_resume_challenge_distribution.arn]
    }
  }
}


resource "aws_cloudfront_origin_access_control" "default" {
  name                              = "cloud-resume-oac"
  description                       = "Origin Access Control for Cloud Resume Challenge"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

resource "aws_cloudfront_distribution" "cloud_resume_challenge_distribution" {
  origin {
    domain_name              = aws_s3_bucket.cloud_resume_bucket.bucket_regional_domain_name
    origin_access_control_id = aws_cloudfront_origin_access_control.default.id
    origin_id               = "cloud-resume-s3-origin"

  }

  enabled             = true
  is_ipv6_enabled     = true
  default_root_object = "index.html"
  http_version        = "http2and3"

  aliases = [var.domain_name]

  default_cache_behavior {
    allowed_methods  = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "cloud-resume-s3-origin"

    forwarded_values {
      query_string = true

      cookies {
        forward = "none"
      }
    }

    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 3600
    max_ttl                = 86400
  }

  custom_error_response {
    error_caching_min_ttl = 10
    error_code            = 404
    response_code         = 404
    response_page_path    = "/404/index.html"
  }

  price_class = var.price_class

  restrictions {
    geo_restriction {
      restriction_type = "none"
      locations        = []
    }
  }

  viewer_certificate {
    acm_certificate_arn            = var.acm_certificate_arn
    cloudfront_default_certificate = false
    minimum_protocol_version       = "TLSv1.2_2021"
    ssl_support_method             = "sni-only"
  }
}

