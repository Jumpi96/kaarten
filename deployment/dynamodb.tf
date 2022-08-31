resource "aws_dynamodb_table" "collectors-dynamodb-table" {
  name         = "Collectors"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "UserId"
  range_key    = "ChatId"

  attribute {
    name = "UserId"
    type = "N"
  }

  attribute {
    name = "ChatId"
    type = "N"
  }

  point_in_time_recovery {
    enabled = true
  }
  
  tags = {
    Name        = "kaarten-collectors-table"
  }
}
