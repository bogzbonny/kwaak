# Issue Summary

**Title**: {{ title }}

## Description
{{ body }}

## Comments
{% for comment in comments %}
* **{{ comment.user.login }}**: {{ comment.body }}
{% endfor %}
