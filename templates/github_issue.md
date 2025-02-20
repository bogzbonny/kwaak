# GitHub Issue #{{ number }} - {{ title }}

**Status**: {{ state }}

{{ body }}

{% if comments | length > 0 %}
## Discussion
{% for comment in comments %}
**@{{ comment.author }}**:
{{ comment.text }}

{% endfor %}
{% endif %}

## Summary
Issue {% if state == "open" %}requires{% else %}required{% endif %} {{ action }} regarding {{ topic }}. {% if timeline %}Timeline: {{ timeline }}{% endif %}
