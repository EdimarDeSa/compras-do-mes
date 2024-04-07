from django.contrib import admin

from .models import (
    DefaultCategorys,
    DefaultProducts,
    Market,
    ProductList,
    ShoppingList,
    ShoppingLog,
    UnityTypes,
    UserCategorys,
    UserProducts,
    Users,
)

# Register your models here.
admin.site.register(Users)
admin.site.register(DefaultCategorys)
admin.site.register(DefaultProducts)
admin.site.register(UnityTypes)
admin.site.register(Market)
admin.site.register(UserCategorys)
admin.site.register(UserProducts)
admin.site.register(ShoppingList)
admin.site.register(ShoppingLog)
admin.site.register(ProductList)
