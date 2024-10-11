package com.ajila.detectors;

import com.ajila.ElementType;

import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.text.PDFTextStripper;

import java.io.IOException;
import java.util.Map;

/**
 * @author fboesiger
 */
public class WordsDetector extends Detector {
    @Override
    public void addElements(PDDocument document, Map<ElementType, Integer> elements) throws IOException {
        PDFTextStripper reader = new PDFTextStripper();
        String text = reader.getText(document);
        String trim = text.trim();

        if (trim.isEmpty()) {
            return;
        }

        int numWords = trim.split("\\s+").length;

        elements.compute(ElementType.WORD, (k, v) -> v == null ? numWords : v + numWords);
    }
}
