import java.util.HashMap;

public class ImprovedHashmap {

    private HashMap<Long, Long> map;
    private long max;
    private long min;

    public ImprovedHashmap() {
        map = new HashMap<>();
    }
    void plus(long key) {
        if (!map.containsKey(key)){
            map.put(key, 0L);
        }
        map.put(key, map.get(key)+1);
    }
    void minus(long key) {
        if (!map.containsKey(key)){
            return;
        }
        if (map.get(key) == 1) {
            map.remove(key);
        } else {
            map.put(key, map.get(key) - 1);
        }
    }


}
