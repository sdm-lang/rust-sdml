{%- macro cardinality(member) %}`{% if member.cardinality.min_occurs == member.cardinality.max_occurs %}{{ member.cardinality.min_occurs }}{% else %}{{ member.cardinality.min_occurs }}..{{ member.cardinality.max_occurs }}{% endif %}`{% endmacro cardinality -%}

{%- macro ordering(member) %}{% if member.cardinality.ordering %}`{{ member.cardinality.ordering }}`{% else %}*default*{% endif %}{% endmacro ordering -%}

{%- macro uniqueness(member) %}{% if member.cardinality.uniqueness %}`{{ member.cardinality.uniqueness }}`{% else %}*default*{% endif %}{% endmacro uniqueness -%}

