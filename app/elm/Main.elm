module Main exposing (..)

import Html exposing (Html, div, program)
import Json.Decode as Decode exposing (Value)
import Navigation exposing (Location)
import Page.Errored as Errored exposing (PageLoadError)
import Page.NotFound as NotFound
import Page.Products as Products
import Route exposing (Route)
import Task
import Util exposing ((=>))
import Views.Page as Page exposing (ActivePage)


-- import Page.Product as Product


type Page
    = Blank
    | NotFound
    | Errored PageLoadError
    | ProductList Products.Model


type PageState
    = Loaded Page
    | TransitioningFrom Page



---- MODEL ----


type alias Model =
    { pageState : PageState }


{-| Set the location of
a page depending on the URL
-}
init : Value -> Location -> ( Model, Cmd Msg )
init val location =
    setRoute (Route.fromLocation location)
        { pageState = Loaded initialPage }


{-| initial page should be blank
-}
initialPage : Page
initialPage =
    Blank



---- VIEW ----


view : Model -> Html Msg
view model =
    case model.pageState of
        Loaded page ->
            viewPage False page

        TransitioningFrom page ->
            viewPage True page


viewPage : Bool -> Page -> Html Msg
viewPage isLoading page =
    let
        -- partially apply the frame function
        -- it takes 2 more args, but we always
        -- want it to reflect the current "isLoading"
        -- no matter.
        frame =
            Page.frame isLoading
    in
    -- fully apply frame
    case page of
        Blank ->
            Html.text ""
                |> frame Page.Other

        NotFound ->
            NotFound.view
                |> frame Page.Other

        Errored subModel ->
            Errored.view subModel
                |> frame Page.Other

        ProductList subModel ->
            Products.view subModel
                |> frame Page.Products



---- SUBS ----


getPage : PageState -> Page
getPage pageState =
    case pageState of
        Loaded page ->
            page

        TransitioningFrom page ->
            page



---- UPDATE ----


type Msg
    = SetRoute (Maybe Route)
    | RootLoaded (Result PageLoadError Products.Model)


setRoute : Maybe Route -> Model -> ( Model, Cmd Msg )
setRoute maybeRoute model =
    let
        transition toMsg task =
            { model | pageState = TransitioningFrom (getPage model.pageState) }
                => Task.attempt toMsg task

        errored =
            pageErrored model
    in
    case maybeRoute of
        Nothing ->
            { model | pageState = Loaded NotFound } => Cmd.none

        Just Route.Root ->
            transition RootLoaded Products.init

        Just Route.Products ->
            -- redirect to Route.Root
            model => Route.modifyUrl Route.Root


pageErrored : Model -> ActivePage -> String -> ( Model, Cmd msg )
pageErrored model activePage errorMessage =
    let
        error =
            Errored.pageLoadError activePage errorMessage
    in
    { model | pageState = Loaded (Errored error) } => Cmd.none


{-| update for our main program. Within the function
we call seperate stuff like updatePage to update the page
itself
-}
update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    updatePage (getPage model.pageState) msg model


{-| update the page
-}
updatePage : Page -> Msg -> Model -> ( Model, Cmd Msg )
updatePage page msg model =
    let
        -- session =
        --     model.session
        toPage toModel toMsg subUpdate subMsg subModel =
            let
                ( newModel, newCmd ) =
                    subUpdate subMsg subModel
            in
            ( { model | pageState = Loaded (toModel newModel) }, Cmd.map toMsg newCmd )

        errored =
            pageErrored model
    in
    case ( msg, page ) of
        ( SetRoute route, _ ) ->
            setRoute route model

        ( RootLoaded (Ok subModel), _ ) ->
            { model | pageState = Loaded (ProductList subModel) } => Cmd.none

        ( _, NotFound ) ->
            -- Disregard incoming messages when we're on the
            -- NotFound page.
            model => Cmd.none

        ( _, _ ) ->
            -- Disregard incoming messages that arrived for the wrong page
            model => Cmd.none



---- PROGRAM ----


main : Program Value Model Msg
main =
    Navigation.programWithFlags (Route.fromLocation >> SetRoute)
        { init = init
        , view = view
        , update = update
        , subscriptions = always Sub.none
        }
