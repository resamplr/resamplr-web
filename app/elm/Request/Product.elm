module Request.Product exposing (..)

import Data.Product exposing (Product, decoder)
import Http
import Json.Decode as Decode
import Request.Helpers exposing (apiUrl)
import Util exposing ((=>))


index : Http.Request (List Product)
index =
    Decode.list decoder
        |> Http.get (apiUrl "/products")
