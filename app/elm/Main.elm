module Main exposing (..)

import Html exposing (Html, button, div, node, program, text)
import Html.Attributes exposing (attribute)
import Html.Events exposing (onClick)
import Http
import Json.Decode as Decode
import Task


type alias Product =
    { id : Int
    , name : String
    , description : String
    , price_cents : Int
    }


type alias Products =
    List Product



---- MODEL ----


type alias Model =
    { products : Products }


init : ( Model, Cmd Msg )
init =
    ( Model [], getProducts )



---- UPDATE ----


type Msg
    = NoOp
    | ProductsGet
    | ProductsFetch (Result Http.Error Products)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ProductsFetch (Ok p) ->
            ( { model | products = p }, Cmd.none )

        ProductsGet ->
            ( model, getProducts )

        _ ->
            ( model, Cmd.none )



---- VIEW ----


view : Model -> Html Msg
view model =
    div []
        [ viewProducts model.products
        , button [ onClick ProductsGet ] [ text "Refresh" ]
        ]


viewProducts : Products -> Html Msg
viewProducts products =
    div []
        (List.map
            (\p ->
                text p.name
            )
            products
        )



---- PROGRAM ----


main : Program Never Model Msg
main =
    Html.program
        { view = view
        , init = init
        , update = update
        , subscriptions = always Sub.none
        }



-- HTTP --


getProducts : Cmd Msg
getProducts =
    let
        url =
            "https://resamplr.com/api/v1/products"
    in
    Http.send ProductsFetch (Http.get url decodeProducts)


decodeProducts : Decode.Decoder Products
decodeProducts =
    Decode.list decodeProduct


decodeProduct : Decode.Decoder Product
decodeProduct =
    Decode.map4 Product
        (Decode.field "id" Decode.int)
        (Decode.field "name" Decode.string)
        (Decode.field "description" Decode.string)
        (Decode.field "price_cents" Decode.int)
