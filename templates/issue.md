# Issue Summary: {{ title }}

**Issue #{{ number }}** - [View on GitHub]({{ html_url }})
**Status**: {{ state }} {% if assignee %}| **Assignee**: {{ assignee }}{% endif %}

## Description
{{ body }}

{% if comments > 0 %}
## Discussion
This issue has {{ comments }} comment(s).
{% endif %}

{% if labels %}
## Labels
{% for label in labels %}
- {{ label.name }}
{% endfor %}
{% endif %}

---
Created by {{ user.login }} | Last updated: {{ updated_at }}
