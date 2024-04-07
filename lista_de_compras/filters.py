from django_filters import rest_framework as filters

from .models import *


class DefaultProductsFilter(filters.FilterSet):
    default_categorys_id = filters.ChoiceFilter(
        field_name="default_categorys_id",
        label="Categoria",
        choices=DefaultCategorys.objects.all().values_list("_id", "name")
    )

    class Meta:
        model = DefaultProducts
        fields = {"name": ["icontains"], "default_categorys_id": ["exact"]}

