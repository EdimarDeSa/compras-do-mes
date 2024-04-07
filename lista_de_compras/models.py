import uuid

from django.db import models

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
]


# Create your models here.
class Users(models.Model):
    _id = models.UUIDField(
        primary_key=True, default=uuid.uuid4, editable=False, db_column="id"
    )
    nickname = models.CharField(max_length=255, verbose_name="Nome", null=False)
    email = models.EmailField(
        max_length=255,
        verbose_name="E-mail",
        null=False,
        unique=True,
        serialize=True,
    )
    password = models.CharField(
        max_length=255, verbose_name="Senha", null=False
    )
    birth_date = models.DateField(
        verbose_name="Data de aniversário", null=True, blank=True
    )
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def __str__(self) -> str:
        return str(self.nickname)

    class Meta:
        db_table = "users"
        verbose_name = "Usuário"
        verbose_name_plural = "Usuários"
        ordering = ["nickname"]


class DefaultCategorys(models.Model):
    _id = models.AutoField(primary_key=True, db_column="id")
    name = models.CharField(max_length=100, null=False, unique=True)

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "default_categorys"
        verbose_name = "Categoria"
        verbose_name_plural = "Categorias"
        ordering = ["name"]


class UnityTypes(models.Model):
    _id = models.AutoField(primary_key=True, db_column="id")
    name = models.CharField(max_length=25, null=False, unique=True)
    abbreviation = models.CharField(max_length=5, null=False, unique=True)
    factor = models.PositiveSmallIntegerField(
        null=False, verbose_name="Base de calculo", serialize=True
    )

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "unity_types"
        verbose_name = "Tipo de unidade"
        verbose_name_plural = "Tipos de unidades"
        ordering = ["name"]


class DefaultProducts(models.Model):
    _id = models.AutoField(primary_key=True, db_column="id")
    name = models.CharField(max_length=100, null=False, unique=True)
    unity_types_id = models.ForeignKey(UnityTypes, on_delete=models.CASCADE)
    default_categorys_id = models.ForeignKey(
        DefaultCategorys, on_delete=models.CASCADE
    )
    image_url = models.URLField(null=True, blank=True, verbose_name="Image URL")

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "default_products"
        verbose_name = "Produto"
        verbose_name_plural = "Produtos"
        ordering = ["name"]


class Market(models.Model):
    _id = models.UUIDField(
        primary_key=True, default=uuid.uuid4, editable=False, db_column="id"
    )
    name = models.CharField(
        max_length=255, verbose_name="Nome", null=False, unique=True
    )
    user_id = models.ForeignKey(Users, on_delete=models.CASCADE)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "markets"
        verbose_name = "Mercado"
        verbose_name_plural = "Mercados"
        ordering = ["name"]


class UserCategorys(models.Model):
    _id = models.UUIDField(
        primary_key=True, default=uuid.uuid4, editable=False, db_column="id"
    )
    name = models.CharField(max_length=255, null=False, unique=True)
    user_id = models.ForeignKey(Users, on_delete=models.CASCADE)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "user_categorys"
        verbose_name = "Categoria"
        verbose_name_plural = "Categorias"
        ordering = ["name"]


class UserProducts(models.Model):
    _id = models.UUIDField(
        primary_key=True, default=uuid.uuid4, editable=False, db_column="id"
    )
    name = models.CharField(max_length=255, null=False, unique=True)
    unity_types_id = models.ForeignKey(UnityTypes, on_delete=models.CASCADE)
    price = models.PositiveBigIntegerField(
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
    user_id = models.ForeignKey(Users, on_delete=models.CASCADE)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "user_products"
        verbose_name = "Produto"
        verbose_name_plural = "Produtos"
        ordering = ["name"]


class ShoppingList(models.Model):
    _id = models.UUIDField(
        primary_key=True, default=uuid.uuid4, editable=False, db_column="id"
    )
    name = models.CharField(max_length=255, null=False)
    user_id = models.ForeignKey(Users, on_delete=models.CASCADE)
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
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def __str__(self) -> str:
        return str(self.name)

    class Meta:
        db_table = "shopping_list"
        verbose_name = "Lista de compras"
        verbose_name_plural = "Listas de compras"
        ordering = ["name"]


class ShoppingLog(models.Model):
    _id = models.UUIDField(
        primary_key=True, default=uuid.uuid4, editable=False, db_column="id"
    )
    user_id = models.ForeignKey(Users, on_delete=models.CASCADE)
    shopping_list_id = models.ForeignKey(ShoppingList, on_delete=models.CASCADE)
    market_id = models.ForeignKey(Market, on_delete=models.CASCADE)
    buy_date = models.DateTimeField(
        auto_now_add=True, verbose_name="Data da compra"
    )
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def __str__(self) -> str:
        return f"{self.market_id.name} - {self.shopping_list_id.name}"

    class Meta:
        db_table = "shopping_log"
        verbose_name = "Log de compras"
        verbose_name_plural = "Logs de compras"
        ordering = ["created_at"]


class ProductList(models.Model):
    _id = models.UUIDField(
        primary_key=True, default=uuid.uuid4, editable=False, db_column="id"
    )
    shopping_list_id = models.ForeignKey(ShoppingList, on_delete=models.CASCADE)
    user_product_id = models.ForeignKey(UserProducts, on_delete=models.CASCADE)
    quantity = models.PositiveSmallIntegerField(
        verbose_name="Quantidade", default=1
    )
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
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "product_list"
        verbose_name = "Item da lista"
        verbose_name_plural = "Itens da lista"
        ordering = ["created_at"]
