from django.apps import AppConfig


class ListaDeComprasConfig(AppConfig):
    default_auto_field = "django.db.models.BigAutoField"
    name = "lista_de_compras"

    def ready(self) -> None:
        import lista_de_compras.signals

        return super().ready()
