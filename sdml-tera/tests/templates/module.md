# Module `{{ module.name }}`

{% if module.base_uri or module.version_info or module.version_uri%}| **Metadata Key** | **Value** |
{% if module.base_uri %}| Base URI | <{{ module.base_uri }}> |{% endif %}
{% if module.version_info %}| version Info | "{{ module.version_info }}" |{% endif %}
{% if module.version_uri %}| version URI | <{{ module.version_uri }}> |{% endif %}{% endif %}

{% if module.annotations %}| **Annotation Name** | **Type** | **Value** |{% for annotation in module.annotations %}{% if annotation.__type == "property" %}
| {{ annotation.name }} | {{ annotation.value.__type }} | {{ annotation.value.value }}|
{% endif %}{% endfor %}{% endif %}

## Dependencies

{% if module.imports %}{% for import in module.imports %}
* {{ import.module }}{% if import.member %}:{{ import.member }}{% endif %}{% if import.version_uri %} @ <{{import.version_uri}}>{% endif %}{% endfor %}{% endif %}
