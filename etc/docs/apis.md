# APIs


## Health API

- API Path: GET `/health/`

### Success Response

```json
{
  "success": true,
  "data": {
    "env": "dev"
  }
}
```

### Error Response

```json
{
  "success": false,
  "message": "not able to connect with database/<or some other message error>"
}
```



## Send Chained Cash ( Online Channel )

`Description: User sends Chained Cash to another User, via the Online Channel.`


- API URL: POST /v1/api/send/cash/offline/

### Request

```json
{
    "from_did": "String",
    "to_did": "String",
    "certificates": "serde_json::Value",
    "transactions": "serde_json::Value",
    // sign and hash ON HOLD.
}
```

### Response

#### Success
```json
{
  "data": {},
  "success": true
}
```

#### Error
```json
{
  "message":"failure message",
  "success":false
}
```

## Receive Chained Cash ( Online Channel )

`Description: User receives Chained cash from another User, via the Online Channel.`


- API URL: POST /v1/api/recv/cash/offline/

### Request

```json
{
    "version": "i64",
    "did": "String",
}
```

### Response

#### Success
```json
{
  "data": {
    "version": "i64",
    "certificates": "Vec<serde_json::Value>",
    "transactions": "Vec<serde_json::Value>",
  },
  "success": true
}
```

#### Error
```json
{
  "message":"failure message",
  "success":false
}
```



## Set Status for Received Chained Cash ( Online Channel )

`Description: User sets the status for received Chained cash.`


- API URL: POST /v1/api/recv/status/

### Request

```json
{
    "version": "i64",
    "did": "String",
}
```

### Response

#### Success
```json
{
  "data": null,
  "success": true
}
```

#### Error
```json
{
  "message":"failure message",
  "success":false
}
```





-----------------------------------------

## Success Response

```json
{
  "success": true,
  "data": "<response payload>",
}
```


## Error Codes

- 404: Not Found
- 500: Internet Server Error
- 400: Bad Request
- 401: Unauthorized
- 200: Success
- 201: Created
