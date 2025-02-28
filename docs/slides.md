---
theme: seriph
title: Castle Clicker
class: text-center
drawings:
  persist: false
transition: slide-left
background: ./background.png
mdc: true
---

# Castle Clicker

Финальный проект для курса Otus "Rust Developer. Professional"


---

# О чем проект

Кликер и idle менеджер в терминале про развитие замка

- 📝 **TUI** - текстовый интерфейс в терминале
- 💎 **золото** - основная валюта, накапливается при клике на пробел
- 🛡️ **оборона** - очки защиты замка - игра заканичвается если они становятся равны 0
- 🔔 **события** - система которая генерирует события каждые 2.5 секунды
- 🛠️ **улучшения** - улучшения для замка, покупаются за золото. разово увеличивают оборону, либо повышают скорость генерации золота или обороны
- 🌐 **мультиязычность** - поддержка нескольких языков

<style>
h1 {
  background-color: #2B90B6;
  background-image: linear-gradient(45deg, #4EC5D4 10%, #146b8c 20%);
  background-size: 100%;
  -webkit-background-clip: text;
  -moz-background-clip: text;
  -webkit-text-fill-color: transparent;
  -moz-text-fill-color: transparent;
}
</style>

---

# Технологии

- 🎨 **ratatui** - библиотека для создания TUI приложений
- 📢 **bevy_ecs** - библиотека для создания ECS систем
- 📖 **rust-i18n** - библиотека для работы с локализацией
- 📦 **serde** и **toml** - библиотеки для сериализации и десериализации TOML конфига

<br>

<style>
h1 {
  background-color: #2B90B6;
  background-image: linear-gradient(45deg, #4EC5D4 10%, #146b8c 20%);
  background-size: 100%;
  -webkit-background-clip: text;
  -moz-background-clip: text;
  -webkit-text-fill-color: transparent;
  -moz-text-fill-color: transparent;
}
</style>

---
drawings:
  persist: false
class: text-center
---

# А теперь демо

<img src="./demo.png" />
