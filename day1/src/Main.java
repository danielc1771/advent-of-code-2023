import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

public class Main {
    public static void main(String[] args) {
        var numsMap = new HashMap<String, String>();
        numsMap.put("one", "1");
        numsMap.put("two", "2");
        numsMap.put("three", "3");
        numsMap.put("four", "4");
        numsMap.put("five", "5");
        numsMap.put("six", "6");
        numsMap.put("seven", "7");
        numsMap.put("eight", "8");
        numsMap.put("nine", "9");

        int ans = 0;
        try {
            FileReader reader = new FileReader("./input");
            BufferedReader bufferedReader = new BufferedReader(reader);

            String line;

            while((line = bufferedReader.readLine()) != null) {
                ans += getValue(line, numsMap);
            }

            System.out.println(ans);
            bufferedReader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static int getValue(String line, Map<String, String> numsMap) {

        int start = 0;
        int end = 0;
    
        var numberChars = new ArrayList<Character>();

        for (int i = 0; i < line.length(); i++) {
            if (Character.isDigit(line.charAt(i))) {
                numberChars.add(line.charAt(i));
                start = i + 1;
                end = i + 1;
                continue;
            }

            var currentWord = line.substring(start, end + 1);

            var updated = false;
            for (var key: numsMap.keySet()) {
                if (currentWord.equals(key)) {
                    numberChars.add(numsMap.get(key).charAt(0));
                    start = end;
                    i = start - 1;
                    updated = true;
                    break;
                } else if (key.startsWith(currentWord)) {        
                    end += 1;
                    updated = true;
                    break;
                }
            }

            if (!updated) {
                i = start;
                start = start + 1;
                end = start;
            }
        }


        System.out.println(line + "->" + numberChars);

        return numberChars.size() > 0 ? Integer.parseInt("" + numberChars.get(0) + numberChars.get(numberChars.size() - 1)) : 0;
    }
}