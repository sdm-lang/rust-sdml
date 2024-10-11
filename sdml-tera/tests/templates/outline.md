{% macro member(item) %}
{%- if item.__type == "reference" -%}
*{{ item.type_ref }}*
{% elif item.__type == "definition" -%}
{{ item.name }} -> *{{ item.type_ref }}*
{% endif -%}
{% endmacro member %}

# Module `{{ module.name }}` Outline

* **{{ module.name }}** (Module)
{% for def in module.definitions %}  * **{{ def.name }}**
{%- if def.__type == "datatype" %} <- *{{ def.base_type }}*
{%- endif %} ({{ def.__type | capitalize | replace(from="-", to=" ") }})
{% if def.__type == "entity" -%}
{%- if def.identity %}    * identity {{ self::member(item=def.identity) }}
{%- endif -%}
{%- if def.members -%}
{% for member in def.members %}    * {{ self::member(item=member) }}
{%- endfor -%}
{%- endif -%}
{%- elif def.__type == "enum" -%}
{% for var in def.variants %}    * {{ var.name }}
{% endfor -%}
{% elif def.__type == "event" -%}
{%- if def.members -%}
{% for member in def.members %}    * {{ self::member(item=member) }}
{%- endfor -%}
{%- endif -%}
{% elif def.__type == "structure" -%}
{%- if def.identity %}  * identity {{ self::member(item=def.identity) }}
{%- endif -%}
{%- if def.members -%}
{% for member in def.members %}    * {{ self::member(item=member) }}
{%- endfor -%}
{%- endif -%}
{%- elif def.__type == "union" -%}
{% for var in def.variants %}    * {% if var.rename %}{{ var.rename }} ({{ var.name }}){% else %}{{ var.name }}{% endif %}
{% endfor -%}
{% endif -%}
{% endfor %}
