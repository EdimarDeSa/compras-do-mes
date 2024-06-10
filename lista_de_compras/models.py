import uuid

from django.contrib.auth.models import AbstractUser
from django.contrib.auth.hashers import make_password
from django.core.exceptions import ValidationError
from django.db import models

from .password_policy import PasswordPolicy

__all__ = [
    "DefaultCategorys",
    "DefaultProducts",
    "Market",
    "ProductList",
    "ShoppingList",
    "ShoppingLog",
    "UnityTypes",
    "UserCategorys",
    "UserProducts",
    "Users",
    "UserProperty",
]


class DefaultCategorys(models.Model):
    _id = models.AutoField(primary_key=True, db_column="id")
    name = models.CharField(max_length=100, null=False, unique=True)

    objects = models.Manager()

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "default_categorys"
        verbose_name = "Categoria padrão"
        verbose_name_plural = "Categorias padrão"
        ordering = ["name"]


class UnityTypes(models.Model):
    _id = models.AutoField(primary_key=True, db_column="id")
    name = models.CharField(max_length=25, null=False, unique=True)
    abbreviation = models.CharField(max_length=5, null=False, unique=True)
    factor = models.PositiveSmallIntegerField(
        null=False, verbose_name="Base de calculo", serialize=True
    )

    objects = models.Manager()

    def __str__(self) -> str:
        return str(self.abbreviation)

    class Meta:
        db_table = "unity_types"
        verbose_name = "Tipo de unidade"
        verbose_name_plural = "Tipos de unidades"
        ordering = ["name"]


class DefaultProducts(models.Model):
    _id = models.AutoField(primary_key=True, db_column="id")
    name = models.CharField(max_length=100, null=False, unique=True)
    unity_types_id = models.ForeignKey(UnityTypes, on_delete=models.CASCADE)
    default_categorys_id = models.ForeignKey(DefaultCategorys, on_delete=models.CASCADE)
    image_url = models.URLField(null=True, blank=True, verbose_name="Image URL")

    objects = models.Manager()

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "default_products"
        verbose_name = "Produto padrão"
        verbose_name_plural = "Produtos padrão"
        ordering = ["name"]


class Users(AbstractUser):
    _id = models.UUIDField(
        primary_key=True, default=uuid.uuid4, editable=False, db_column="id"
    )
    nickname = models.CharField(max_length=255, verbose_name="Apelido", null=False)
    email = models.EmailField(
        max_length=255,
        verbose_name="E-mail",
        null=False,
        unique=True,
        serialize=True,
    )
    password = models.CharField(max_length=255, verbose_name="Senha", null=False)
    birth_date = models.DateField(
        verbose_name="Data de aniversário", null=True, blank=True
    )
    is_active = models.BooleanField(default=False, verbose_name="Usuário ativo")

    def validate_password(self) -> None:
        policy = PasswordPolicy(str(self.password))
        policy.validate()

        if not policy.is_valid:
            errors = {
                err_name: ValidationError(message, code="invalid")
                for err_name, message in policy.errors.items()
            }
            raise errors

    def save(self, *args, **kwargs):
        super().save(*args, **kwargs)
        self.activate_new_user()

    def activate_new_user(self) -> None:
        if self.is_superuser or self.is_staff:
            return

        if self.is_active:
            return

        categorys = DefaultCategorys.objects.all()
        for category in categorys:
            UserCategorys.objects.create(
                name=category.name,
                user_id=self,
            ).save()

        products = DefaultProducts.objects.all()
        for product in products:
            UserProducts.objects.create(
                name=product.name,
                unity_types_id=product.unity_types_id,
                price_unity_types_id=product.unity_types_id,
                category_id=UserCategorys.objects.get(
                    name=product.default_categorys_id.name,
                    user_id=self,
                ),
                user_id=self,
            ).save()

        self.is_active = True
        super().save()

    def __str__(self) -> str:
        return str(self.username)

    class Meta:
        db_table = "users"
        ordering = ["nickname"]


class CommonInfo(models.Model):
    """Classe abstrata para adicionar campos de data de criação e atualização.
    _id: UUID -> Campo de identificação do registro.
    created_at: DateTime -> Data de criação do registro.
    updated_at: DateTime -> Data de atualização do registro.
    """

    _id = models.UUIDField(
        primary_key=True, default=uuid.uuid4, editable=False, db_column="id"
    )
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    objects = models.Manager()

    class Meta:
        abstract = True


class UserProperty(CommonInfo):
    """Classe abstrata para adicionar o usuário dono do registro."""

    user_id = models.ForeignKey(Users, on_delete=models.CASCADE)

    class Meta:
        abstract = True


class Market(UserProperty):
    name = models.CharField(max_length=255, verbose_name="Nome", null=False)

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "markets"
        verbose_name = "Mercado do usuário"
        verbose_name_plural = "Mercados do usuário"
        ordering = ["name"]
        unique_together = (("name", "user_id"),)


class UserCategorys(UserProperty):
    name = models.CharField(max_length=255, null=False)

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "user_categorys"
        verbose_name = "Categoria do usuário"
        verbose_name_plural = "Categorias do usuário"
        ordering = ["name"]
        unique_together = (("name", "user_id"),)


class UserProducts(UserProperty):
    name = models.CharField(max_length=255, null=False)
    unity_types_id = models.ForeignKey(UnityTypes, on_delete=models.CASCADE)
    price = models.FloatField(
        verbose_name="Preço",
        default=0,
        help_text="Sempre deve ser armazenado em formato inteiro!",
    )
    price_unity_types_id = models.ForeignKey(
        UnityTypes,
        on_delete=models.CASCADE,
        related_name="price_unity_types_id",
    )
    category_id = models.ForeignKey(UserCategorys, on_delete=models.CASCADE)
    notes = models.CharField(max_length=255, null=True, blank=True)
    barcode = models.CharField(max_length=50, null=True, blank=True)
    image_url = models.URLField(null=True, blank=True, verbose_name="Image URL")

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "user_products"
        verbose_name = "Produto do usuário"
        verbose_name_plural = "Produtos do usuário"
        ordering = ["name"]
        unique_together = (("name", "user_id"),)


class ShoppingList(UserProperty):
    name = models.CharField(max_length=255, null=False)
    final_value = models.PositiveBigIntegerField(
        verbose_name="Valor final",
        default=0,
        help_text="Sempre deve ser armazenado em formato inteiro!",
    )
    unique_items = models.PositiveSmallIntegerField(
        verbose_name="Itens únicos", default=0
    )
    total_items = models.PositiveSmallIntegerField(
        verbose_name="Total de itens", default=0
    )

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "shopping_list"
        verbose_name = "Lista de compras do usuário"
        verbose_name_plural = "Listas de compras do usuário"
        ordering = ["name"]
        unique_together = (("name", "user_id"),)


class ShoppingLog(UserProperty):
    shopping_list_id = models.ForeignKey(ShoppingList, on_delete=models.CASCADE)
    market_id = models.ForeignKey(Market, on_delete=models.CASCADE)
    buy_date = models.DateTimeField(auto_now_add=True, verbose_name="Data da compra")

    def __str__(self) -> str:
        return f"{self.market_id.name} - {self.shopping_list_id.name}"

    class Meta:
        db_table = "shopping_log"
        verbose_name = "Histórico de compras do usuário"
        verbose_name_plural = "Históricos de compras do usuário"
        ordering = ["created_at"]


class ProductList(CommonInfo):
    shopping_list_id = models.ForeignKey(ShoppingList, on_delete=models.CASCADE)
    user_product_id = models.ForeignKey(UserProducts, on_delete=models.CASCADE)
    quantity = models.PositiveSmallIntegerField(verbose_name="Quantidade", default=1)
    price = models.PositiveBigIntegerField(
        verbose_name="Preço",
        default=0,
        help_text="Sempre deve ser armazenado em formato inteiro!",
    )
    total = models.PositiveBigIntegerField(
        verbose_name="Total",
        default=0,
        help_text="Sempre deve ser armazenado em formato inteiro!",
    )
    on_cart = models.BooleanField(default=False, verbose_name="No carrinho")

    class Meta:
        db_table = "product_list"
        verbose_name = "Item da lista"
        verbose_name_plural = "Itens da lista"
        ordering = ["created_at"]
