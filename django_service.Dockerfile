# syntax=docker/dockerfile:1
FROM python:latest
LABEL authors="edimar"

ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

WORKDIR /app

COPY . .

RUN pip install -r requirements.txt

CMD python manage.py makemigrations && python manage.py migrate && python manage.py runserver 0.0.0.0:8000
