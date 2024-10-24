{% import "macros.md" as macros -%}
# Module {{ module.name }} dimensions

{%- for defn in module.definitions -%}
  {%- if defn.__type == "dimension" -%}
## Dimension DIM_{{ defn.name | snake_case | upper }} ({{ defn.name }})

### Keys
    {% if defn.identity.__type == "source_entity" %}
* `DIM_{{ defn.identity.entity | snake_case | upper }}_ID` (primary)
    {% elif defn.identity.__type == "definition" %}
* `{{ defn.identity.name | snake_case | upper }}`
    {% endif %}
    {% if defn.parents %}
### Parents

| Parent | Accessor |
|--------|----------|
      {%- for parent in defn.parents %}
| `DIM_{{ parent.entity | snake_case | upper }}` | `{{ parent.name }}` |
      {% endfor -%}
    {% endif -%}
    {% if defn.members %}
### Members

| Name | Type | Cardinality | Ordering | Uniqueness |
|------|------|-------------|----------|------------|
      {%- for member in defn.members %}
| `{{ member.name | snake_case | upper }}` | `{{ member.type_ref }}` | {{ macros::cardinality(member=member) }} | {{ macros::ordering(member=member) }} | {{ macros::uniqueness(member=member) }} |
      {% endfor %}
    {% endif %}
  {% endif %}
{% endfor %}
