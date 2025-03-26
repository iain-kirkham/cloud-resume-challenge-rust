resource "aws_dynamodb_table" "cloud-resume-challenge" {
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "ID"
  name         = "cloud-resume-challenge"
  table_class  = "STANDARD"
  attribute {
    name = "ID"
    type = "S"
  }
}