import re

from rest_framework.serializers import ValidationError


def validate_password(value):
    if len(value) < 8:
        raise ValidationError("A senha deve conter no mínimo 8 caracteres.")

    if not re.search(r"\d", value):
        raise ValidationError("A senha deve conter no mínimo 1 número.")

    if not re.search(r"[A-Z]", value):
        raise ValidationError("A senha deve conter no mínimo 1 letra maiúscula.")

    if not re.search(r"[a-z]", value):
        raise ValidationError("A senha deve conter no mínimo 1 letra minúscula.")

    if not re.search(r"[!@#$%^&*(),.?\":{}|<>]", value):
        raise ValidationError("A senha deve conter no mínimo 1 caractere especial.")
