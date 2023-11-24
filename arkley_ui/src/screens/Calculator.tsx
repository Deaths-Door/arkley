import { View , StyleSheet} from "react-native";
import Keyboard from "../components/Keyboard";
import { useState } from "react";
import { Text } from "react-native-paper" 

export default function Calculator() {
    const [input,setInput] = useState("0");
    
    return <>
        { /* TODO : Remove this one below its usless */}
        <View style={{height : 180 }}></View>
        <View>
            <Text style={styles.text} variant="headlineSmall">{input}</Text>
            <Text style={styles.text} variant="displaySmall">{input}</Text>
        </View>
        <Keyboard input={input} setInput={setInput}/>
    </>
}

const styles = StyleSheet.create({
    text : { textAlign : "right"  , paddingBottom : 12 , paddingEnd : 36 }
});