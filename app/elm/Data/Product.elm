module Data.Product exposing (Product, decoder)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Pipeline exposing (decode, required)


type alias Product =
    { id : Int
    , name : String
    , description : String
    , price_cents : Int
    }



-- SERIALIZATION --


decoder : Decoder Product
decoder =
    decode Product
        |> required "id" Decode.int
        |> required "name" Decode.string
        |> required "description" Decode.string
        |> required "price_cents" Decode.int
