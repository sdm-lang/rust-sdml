# Module `{{ module.name }}`

{% if module.base_uri or module.version_info or module.version_uri%}| **Metadata Key** | **Value** |
{% if module.base_uri %}| Base URI | <{{ module.base_uri }}> |{% endif %}
{% if module.version_info %}| version Info | "{{ module.version_info }}" |{% endif %}
{% if module.version_uri %}| version URI | <{{ module.version_uri }}> |{% endif %}{% endif %}

{% if module.annotations %}| **Annotation Name** | **Type** | **Value** |{% for annotation in module.annotations %}{% if annotation.__type == "property" %}
| {{ annotation.name }} | {{ annotation.value.__type }} | {{ annotation.value.value }}|
{% endif %}{% endfor %}{% endif %}

{% if module.imports %}
## Dependencies

{% if module.imports %}{% for import in module.imports %}
* {{ import.module }}{% if import.member %}:{{ import.member }}{% endif %}{% if import.version_uri %} @ <{{import.version_uri}}>{% endif %}{% endfor %}{% endif %}
{% endif %}

{% if module.definitions %}
## Definitions

{% for definition in module.definitions %}
### {{ definition.__type | capitalize | replace(from="-", to=" ") }} `{{ definition.name }}`
{% if definition.__type == "datatype" %}
> ```sdml
> datatype {{ definition.name }} <- {{ definition.base_type }}
> ```
{% elif definition.__type == "entity" %}
> ```sdml
> entity {{ definition.name }}
> ```

#### Members

{% elif definition.__type == "enum" %}
> ```sdml
> enum {{ definition.name }}
> ```

#### Variants

{% for variant in definition.variants %}
{% endfor %}

{% elif definition.__type == "event" %}
> ```sdml
> event {{ definition.name }} source {{ definition.source }}
> ```

#### Members

{% elif definition.__type == "property" %}
> ```sdml
> property {{ definition.name }}
> ```
{% elif definition.__type == "rdf" %}
> ```sdml
> rdf {{ definition.name }}
> ```
{% elif definition.__type == "structure" %}
> ```sdml
> structure {{ definition.name }}
> ```
{% elif definition.__type == "union" %}
> ```sdml
> union {{ definition.name }}
> ```

#### Variants



#### Members

{% endif %}
{% endfor %}

{% endif %}
