# Issue Summary

## Issue details
{{ issue.title }} (#{{ issue.number }}) - Opened by {{ issue.user.login }}

{{ issue.body }}

{% if comments %}
## Comments

{% for comment in comments %}
**{{ comment.user.login }}**: {{ comment.body }}

{% endfor %}
{% endif %}

## Context
This issue is from {{ repository_owner }}/{{ repository }} and contains {{ comments | length }} comments.

## Format for your response:

1. Provide a clear, concise summary of the issue
2. Identify the main tasks or requirements
3. List any blockers or dependencies
4. If discussion occurred, summarize key decisions or outcomes
5. If relevant, outline the suggested implementation approach

Keep the response focused and actionable.
Do not include any introductions, conclusions, or metadata.
