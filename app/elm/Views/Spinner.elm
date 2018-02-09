module Views.Spinner exposing (spinner)

import Html exposing (Attribute, Html, div, li)
import Html.Attributes exposing (class, style)
import Util exposing ((=>))


spinner : Html msg
spinner =
    div [ class "sk-three-bounce", style [ "float" => "left", "margin" => "8px", "background" => "green", "width" => "100px", "height" => "100px" ] ]
        []
