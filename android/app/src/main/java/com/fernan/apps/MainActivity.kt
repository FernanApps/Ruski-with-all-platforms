package com.fernan.apps


import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.fernan.apps.ui.theme.AndroidRustTemplateTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            AndroidRustTemplateTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {

                    Example()
                }
            }
        }
    }
}




@Preview
@Composable
fun Example(){
    var text by remember {
        mutableStateOf("Hello World")
    }
    Column {
        Library.hello("JniRustCalled")?.let { Greeting(it) }

        TextField(value = text, onValueChange = {
            text = it
        } )
        Row {
            Button(onClick = {
                text = Library.encrypt(text)
            }) {
                Text(text = "Encrypt")
            }
            Button(onClick = {
                text = Library.decrypt(text)
            }) {
                Text(text = "Decrypt")
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello_JetpackCompose $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    AndroidRustTemplateTheme {
        Greeting("Android")
    }
}
