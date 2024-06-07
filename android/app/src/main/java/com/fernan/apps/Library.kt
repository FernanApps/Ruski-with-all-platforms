package com.fernan.apps

object Library {
    init {
        System.loadLibrary("library")
    }

    external fun hello(input: String): String?
    external fun encrypt(input: String): String
    external fun decrypt(input: String): String
    
}