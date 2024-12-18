
## API Reference

#### Register user

```http
  POST /api/user/register
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `form` | `string` | **Required**. Form data encoded with `application/x-www-form-urlencoded` |
- If successful redirects to login page, else returns error.
#### Log-in user

```http
  POST /api/user/login
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `form`      | `string` | **Required**. Form data encoded with `application/x-www-form-urlencoded` |

- Attaches HTTP Cookie with session token on successful authorization. If not returns error.

#### Logout user

```http
  GET /api/user/logout
```
- Revokes the session token and removed the HTTP Cookie. Returns error if unauthenticated.


#### Deletes user

```http
  GET /api/iser/delete
```
- Revokes the session and deletes the user account and the associated data.

#### Reserve ticket

```http
  POST /api/ticket/new
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `route`      | `id` | **Required**. Route ID|
