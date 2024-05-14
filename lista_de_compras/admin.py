from django.contrib import admin

from .models import *


@admin.register(Users)
class UsersAdmin(admin.ModelAdmin):
    list_display = (
        "nickname",
        "email",
        "birth_date",
    )
    search_fields = (
        "nickname",
        "email",
    )
    list_filter = ("birth_date",)
    readonly_fields = ("_id",)
    list_per_page = 50
    list_display_links = (
        "nickname",
        "email",
    )
    fieldsets = (
        (
            "Dados cadastrais:",
            {
                "classes": ["extrapretty"],
                "fields": (
                    "_id",
                    "username",
                    "nickname",
                    "email",
                    "birth_date",
                ),
            },
        ),
        (
            "Dados de criação e atualização:",
            {
                "classes": ["collapse"],
                "fields": (
                    "last_login",
                    "is_superuser",
                    "is_staff",
                    "is_active",
                    "date_joined",
                ),
            },
        ),
    )
    ordering = ("username", "email")
    date_hierarchy = "date_joined"


@admin.display(description="Nome do usuário")
def user_nickname(obj: UserProperty):
    user: Users = obj.user_id
    return user.nickname


@admin.register(Market, UserCategorys)
class UserPropertyNameOnlyAdmin(admin.ModelAdmin):
    list_display = (
        "name",
        user_nickname,
    )
    search_fields = ("name",)
    list_filter = ("user_id",)
    readonly_fields = ("_id", "created_at", "updated_at")
    fields = (
        "_id",
        "name",
        "user_id",
        (
            "created_at",
            "updated_at",
        ),
    )
    list_per_page = 50
    ordering = ("user_id", "name")


@admin.display(description="Categoria")
def category_name(obj: UserProducts):
    category: UserCategorys = obj.category_id
    return category.name


@admin.register(UserProducts)
class UserProductsAdmin(admin.ModelAdmin):
    list_display = (
        "name",
        user_nickname,
        category_name,
    )
    search_fields = ("name", user_nickname)
    list_filter = ("category_id", "user_id")
    readonly_fields = ("_id", "created_at", "updated_at")
    fieldsets = (
        (
            "Dados cadastrais:",
            {
                "classes": ["extrapretty"],
                "fields": (
                    "_id",
                    "name",
                    "user_id",
                    "category_id",
                ),
            },
        ),
        (
            "Dados para calculo:",
            {
                "fields": (
                    "unity_types_id",
                    "price",
                    "price_unity_types_id",
                )
            },
        ),
        (
            "Dados extras:",
            {"classes": ["wide"], "fields": ("notes", "barcode", "image_url")},
        ),
        (
            "Dados de criação e atualização:",
            {"classes": ["collapse", "wide"], "fields": ("created_at", "updated_at")},
        ),
    )


admin.site.register(ShoppingList)
admin.site.register(ShoppingLog)
admin.site.register(ProductList)

admin.site.register(DefaultProducts)
admin.site.register(DefaultCategorys)
admin.site.register(UnityTypes)
