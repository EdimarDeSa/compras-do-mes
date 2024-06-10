import re
from typing import TypedDict, Optional


class ValidationType(TypedDict):
    message: str
    validation: re.Pattern
    must_find: bool


Errors = dict[str, str]


class PasswordPolicy:
    """
    A classe PasswordPolicy é responsável por validar a senha do usuário.
    password: str <- Senha do usuário.
    errors: Errors -> Dicionário com as mensagens de erro.
    is_valid: bool -> Indica se a senha é válida.

    Exemplo:
    policy = PasswordPolicy("senha123")
    policy.validate_password()
    print(policy.is_valid)
    print(policy.errors)

    Saída:
    False
    {
        "min_length": "A senha deve conter no mínimo 8 caracteres.",
        "min_uppercase": "A senha deve conter no mínimo 1 letra maiúscula.",
        "min_special": "A senha deve conter no mínimo 1 caractere especial.",
        "no_whitespace": "A senha não pode conter espaços em branco.",
        "no_sequential_chars": "A senha não deve conter números sequenciais.",
    }

    """

    def __init__(
        self,
        password: str,
    ):
        self.__min_length = 8
        self.__min_digits = 1
        self.__min_uppercase = 1
        self.__min_lowercase = 1
        self.__min_special = 1

        self.__validations: dict[str, ValidationType] = {
            "min_length": {
                "message": "A senha deve conter no mínimo 8 caracteres.",
                "validation": re.compile(rf".{{{self.__min_length},}}"),
                "must_find": True,
            },
            "min_digits": {
                "message": "A senha deve conter no mínimo 1 número.",
                "validation": re.compile(rf"\d{{{self.__min_digits},}}"),
                "must_find": True,
            },
            "min_uppercase": {
                "message": "A senha deve conter no mínimo 1 letra maiúscula.",
                "validation": re.compile(rf"[A-Z]{{{self.__min_uppercase},}}"),
                "must_find": True,
            },
            "min_lowercase": {
                "message": "A senha deve conter no mínimo 1 letra minúscula.",
                "validation": re.compile(rf"[a-z]{{{self.__min_lowercase},}}"),
                "must_find": True,
            },
            "min_special": {
                "message": "A senha deve conter no mínimo 1 caractere especial.",
                "validation": re.compile(
                    rf"[!@#$%^&*(),.?\":{{}}|<>]{{{self.__min_special},}}"
                ),
                "must_find": True,
            },
        }

        if not isinstance(password, str):
            raise TypeError("password argument must be a string.")

        self._password = password

        self.errors: Errors = dict()
        self.is_valid = False

    def validate(self) -> None:
        for validation_name, validation_type in self.__validations.items():
            message: str = validation_type.get("message", "")
            validation: re.Pattern = validation_type.get("validation", None)
            must_find: bool = validation_type.get("must_find", None)

            if validation is None or must_find is None:
                continue

            search = re.search(validation, self._password)

            if bool(search) != must_find:
                self.errors[validation_name] = message

        if self.errors:
            return

        self.is_valid = True

    def __repr__(self) -> str:
        return f"PasswordPolicy(is_valid='{self.is_valid}')"
