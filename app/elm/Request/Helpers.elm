module Request.Helpers exposing (apiUrl)


apiUrl path =
    "http://localhost:3000/api/v1" ++ path
