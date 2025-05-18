package ru.udaltsovk.minizord.email

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

/**
 * Главный класс приложения для отправки электронных писем.
 */
@SpringBootApplication
class EmailApplication

/**
 * Точка входа в приложение.
 * @param args Аргументы командной строки.
 */
fun main(args: Array<String>) {
    runApplication<EmailApplication>(*args)
}
