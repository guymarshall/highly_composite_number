import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;

public class File {
    private static final String RESULTS_FILE = "results.txt";

    public static ResultPair readResults() {
        Path path = Path.of(RESULTS_FILE);

        if (Files.exists(path)) {
            try (BufferedReader reader = Files.newBufferedReader(path)) {
                String line;
                while ((line = reader.readLine()) != null) {
                    String[] parts = line.trim().split(",");
                    if (parts.length == 2) {
                        try {
                            int lastNumber = Integer.parseInt(parts[0]);
                            int maxDivisors = Integer.parseInt(parts[1]);
                            return new ResultPair(lastNumber, maxDivisors);
                        } catch (NumberFormatException ignored) {
                            // TODO: Skip malformed line
                        }
                    }
                }
            } catch (IOException e) {
                System.out.println(e.getMessage());
            }
        }

        // Default if file is missing or malformed
        return new ResultPair(0, 0);
    }

    public static void writeResults(int number, int maxDivisors) {
        Path path = Path.of(RESULTS_FILE);

        try (BufferedWriter writer = Files.newBufferedWriter(path,
                StandardOpenOption.CREATE,
                StandardOpenOption.TRUNCATE_EXISTING,
                StandardOpenOption.WRITE)) {

            writer.write(number + "," + maxDivisors);
            writer.newLine();

        } catch (IOException e) {
            System.out.println(e.getMessage());
        }
    }
}
