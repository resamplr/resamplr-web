module Views.Page exposing (ActivePage(..), bodyId, frame)

{-| The frame around a typical page - that is, the header and footer.
-}

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Lazy exposing (lazy2)
import Route exposing (Route)
import Util exposing ((=>))
import Views.Spinner exposing (spinner)


{-| Determines which navbar link (if any) will be rendered as active.

Note that we don't enumerate every page here, because the navbar doesn't
have links for every page. Anything that's not part of the navbar falls
under Other.

-}
type ActivePage
    = Products
    | Other


{-| Take a page's Html and g it with a header and footer.

The caller provides the current user, so we can display in either
"signed in" (rendering username) or "signed out" mode.

isLoading is for determining whether we should show a loading spinner
in the header. (This comes up during slow page transitions.)

-}
frame : Bool -> ActivePage -> Html msg -> Html msg
frame isLoading page content =
    div [ class "page-frame" ]
        [ viewHeader page isLoading
        , content
        , viewFooter
        ]


viewHeader : ActivePage -> Bool -> Html msg
viewHeader page isLoading =
    div [] []


viewFooter : Html msg
viewFooter =
    footer [] []


navbarLink : ActivePage -> Route -> List (Html msg) -> Html msg
navbarLink page route linkContent =
    div [] []


isActive : ActivePage -> Route -> Bool
isActive page route =
    case ( page, route ) of
        ( Products, Route.Products ) ->
            True

        _ ->
            False


{-| This id comes from index.html.

The Feed uses it to scroll to the top of the page (by ID) when switching pages
in the pagination sense.

-}
bodyId : String
bodyId =
    "page-body"
