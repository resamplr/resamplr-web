module Data.Product.Image exposing (Image, decoder)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Pipeline exposing (decode, required)


type alias Image =
    { background : String
    , logo : String
    , full_image : String
    }



-- SERIALIZATION --


decoder : Decoder Image
decoder =
    decode Image
        |> required "background" Decode.string
        |> required "logo" Decode.string
        |> required "full_image" Decode.string
