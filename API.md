
## API Reference

#### Register user

```http
  POST /api/register
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `form` | `string` | **Required**. Form data encoded with `application/x-www-form-urlencoded` |
- If successful redirects to login page, else returns error.
#### Log-in user

```http
  POST /api/login
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `form`      | `string` | **Required**. Form data encoded with `application/x-www-form-urlencoded` |

- Attaches HTTP Cookie with session token on successful authorization. If not returns error.

