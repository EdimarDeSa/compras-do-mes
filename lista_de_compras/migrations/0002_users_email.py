# Generated by Django 5.0.3 on 2024-04-03 00:49

from django.db import migrations, models


class Migration(migrations.Migration):
    dependencies = [
        ("lista_de_compras", "0001_initial"),
    ]

    operations = [
        migrations.AddField(
            model_name="users",
            name="email",
            field=models.EmailField(
                default="email@email.com",
                max_length=255,
                unique=True,
                verbose_name="E-mail",
            ),
            preserve_default=False,
        ),
    ]
